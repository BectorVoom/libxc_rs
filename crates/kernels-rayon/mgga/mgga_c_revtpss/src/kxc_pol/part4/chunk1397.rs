//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1397/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1397(t13045: f64, t3601: f64, t17710: f64, t3720: f64, t1261: f64, t12784: f64, t17669: f64, t17674: f64, t17679: f64, t17684: f64, t17690: f64, t17693: f64, t17696: f64, t17700: f64, t17705: f64, t17709: f64, t3625: f64, t3708: f64, t5287: f64, t5331: f64, t5340: f64, t5407: f64) -> f64 {
    let t17711 = t13045 * t3601;
    let t17712 = t17710 * t17711;
    let t17713 = t3720 * t17712;
    let t17718 = -0.28582678745379824648e-3_f64 * t12784 * t5407 - 0.28582678745379824648e-3_f64 * t3625 * t17669 - 0.14291339372689912324e-3_f64 * t3625 * t17674 - 0.28582678745379824648e-3_f64 * t5340 * t17679 + 0.14291339372689912324e-3_f64 * t5331 * t17684 + 0.23818898954483187207e-3_f64 * t3625 * t17690 + 0.47637797908966374414e-3_f64 * t17693 * t17696 + 0.47637797908966374414e-3_f64 * t1261 * t17700 + 0.42874018118069736972e-3_f64 * t5340 * t17705 + 0.12862205435420921092e-2_f64 * t17709 * t17713 + 0.42874018118069736972e-3_f64 * t3708 * t5287;
    t17718
}
