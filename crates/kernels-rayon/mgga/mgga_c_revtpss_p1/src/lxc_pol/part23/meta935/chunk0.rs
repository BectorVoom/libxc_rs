//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3074/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3074(t422: f64, t81286: f64, t81304: f64, t20473: f64, t5192: f64, t24407: f64, t3520: f64, t1196: f64, t5206: f64, t20391: f64, t20394: f64, t81254: f64, t81257: f64, t81259: f64, t81261: f64, t81264: f64, t81266: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81307 = 0.621814e-1_f64 * (t81286 + t81304) * t422;
    let t81309 = 0.31168546390226634765e3_f64 * t5192 * t20473;
    let t81310 = t3520 * t24407;
    let t81313 = 0.17315859105681463759e2_f64 * t1196 * t81310 * t5206;
    let t81315 = 0.10526802520742363173e2_f64 * t5192 * t20391;
    let t81317 = 0.70178683471615754484e1_f64 * t5192 * t20394;
    let t81318 = t81254 - t81257 - t81259 + t81261 + t81264 - t81266 - t81307 + t81309 - t81313 - t81315 + t81317;
    (t81307, t81309, t81313, t81315, t81317, t81318)
}
