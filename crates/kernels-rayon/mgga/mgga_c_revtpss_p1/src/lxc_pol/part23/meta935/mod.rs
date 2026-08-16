//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta935 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3074;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta935(t422: f64, t81286: f64, t81304: f64, t20473: f64, t5192: f64, t24407: f64, t3520: f64, t1196: f64, t5206: f64, t20391: f64, t20394: f64, t81254: f64, t81257: f64, t81259: f64, t81261: f64, t81264: f64, t81266: f64, t5184: f64, t68680: f64, t1187: f64, t6534: f64, t1757: f64, t58708: f64, t20400: f64, t5198: f64, t20887: f64, t58665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81307, t81309, t81313, t81315, t81317, t81318) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3074(t422, t81286, t81304, t20473, t5192, t24407, t3520, t1196, t5206, t20391, t20394, t81254, t81257, t81259, t81261, t81264, t81266);
        let (t81322, t81326, t81328, t81330, t81333) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3075(t1196, t5184, t68680, t1187, t6534, t1757, t58708, t20400, t5198, t20887, t5192, t58665);
    (t81307, t81309, t81313, t81315, t81317, t81318, t81322, t81326, t81328, t81330, t81333)
}
