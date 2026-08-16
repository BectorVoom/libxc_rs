//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 959/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk959(t36: f64, t4895: f64, t262: f64, t2068: f64, t4928: f64, t2073: f64, t2079: f64, t5249: f64, t1587: f64, t265: f64, t27091: f64, t40901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41043 = t36 * t4895;
    let t41044 = t262 * t41043;
    let t41045 = t2068 * t41044;
    let t41047 = t36 * t4928;
    let t41048 = t262 * t41047;
    let t41049 = t2073 * t41048;
    let t41053 = t2079 * t262 * t36 * t5249;
    let t41055 = t265 * t1587;
    let t41056 = t262 * t41055;
    let t41057 = t2068 * t41056;
    let t41077 = t27091 * t40901;
    (t41043, t41044, t41045, t41047, t41048, t41049, t41053, t41055, t41056, t41057, t41077)
}
