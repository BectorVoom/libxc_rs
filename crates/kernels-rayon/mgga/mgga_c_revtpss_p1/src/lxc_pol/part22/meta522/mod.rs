//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2294;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2295;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2296;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2297;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta522(t1737: f64, t3451: f64, t1160: f64, t5117: f64, t1170: f64, t12511: f64, t12553: f64, t16809: f64, t16832: f64, t16837: f64, t16839: f64, t16842: f64, t16844: f64, t16846: f64, t16945: f64, t16998: f64, t17020: f64, t3454: f64, t435: f64, t5125: f64, t3476: f64, t16868: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t16706: f64, t16727: f64, t16748: f64, t16871: f64, t16876: f64, t16892: f64, t16708: f64, t16710: f64, t16717: f64, t16722: f64, t16735: f64, t16740: f64, t16744: f64, t16908: f64, t16927: f64, t16931: f64, t16933: f64, t12252: f64, t12261: f64, t12263: f64, t12265: f64, t12459: f64, t12460: f64, t16731: f64, t16852: f64, t16855: f64, t16858: f64, t16860: f64, t16863: f64, t16865: f64, t16887: f64, t16890: f64, t16895: f64, t16898: f64, t16901: f64, t16904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17023, t17026, t17029) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2294(t1737, t3451, t1160, t5117, t1170, t12511, t12553, t16809, t16832, t16837, t16839, t16842, t16844, t16846, t16945, t16998, t17020, t3454, t435, t5125);
        let (t17032, t17050, t17052, t17061) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2295(t1737, t3476, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17066, t17075, t17083) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2296(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17085 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2297(t12252, t12261, t12263, t12265, t12459, t12460, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17061, t17066, t17083);
    (t17023, t17026, t17029, t17032, t17050, t17052, t17066, t17075, t17085)
}
