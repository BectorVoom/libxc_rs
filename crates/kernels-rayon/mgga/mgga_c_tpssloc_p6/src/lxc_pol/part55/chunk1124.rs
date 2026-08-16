//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1124/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1124(t1241: f64, t34305: f64, t32519: f64, t8002: f64, t2154: f64, t8087: f64, t3598: f64, t1760: f64, t8897: f64, t1238: f64, t1761: f64, t2121: f64, t2155: f64, t27792: f64, t32482: f64, t34238: f64, t34241: f64, t34244: f64, t34247: f64, t34251: f64, t34254: f64, t34278: f64, t4945: f64, t498: f64, t7283: f64, t8898: f64) -> (f64, f64, f64, f64, f64) {
    let t34306 = t1241 * t34305;
    let t34310 = t32519 * t8002;
    let t34313 = t2154 * t8087;
    let t34314 = t3598 * t34313;
    let t34317 = t8897 * t1760;
    let t34318 = t3598 * t34317;
    let t34321 = 0.16449340668482264365e-1_f64 * t2121 * t34238 - 0.16449340668482264365e-1_f64 * t7283 * t34241 - 0.16449340668482264365e-1_f64 * t7283 * t34244 - 0.16449340668482264365e-1_f64 * t7283 * t34247 - 0.16449340668482264365e-1_f64 * t7283 * t34251 + t34254 * t498 + t34278 * t498 - 2.0_f64 * t27792 * t2155 - t1238 * t34306 - t4945 * t8898 - t32482 * t1761 - 0.54831135561607547883e-2_f64 * t7283 * t34310 + 4.0_f64 * t1238 * t34314 + 2.0_f64 * t1238 * t34318;
    (t34306, t34310, t34314, t34318, t34321)
}
