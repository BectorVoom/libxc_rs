//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2055;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta644(t25641: f64, t82892: f64, t25638: f64, t6735: f64, t23418: f64, t4669: f64, t13765: f64, t23419: f64, t10469: f64, t23470: f64, t3: f64, t82986: f64, t23437: f64, t4630: f64, t82943: f64, t1933: f64, t1937: f64, t3966: f64, t25655: f64, t82895: f64, t25661: f64, t1036: f64, t25664: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88488, t88503, t88513, t88517, t88537) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2055(t25641, t82892, t25638, t6735, t23418, t4669, t13765, t23419, t10469, t23470, t3, t82986);
        let (t88548, t88566, t88569, t88575, t88577, t88582) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2056(t23437, t4630, t25641, t82943, t1933, t1937, t3966, t25655, t82895, t25661, t1036, t25664);
    (t88488, t88503, t88513, t88517, t88537, t88548, t88566, t88569, t88575, t88577, t88582)
}
