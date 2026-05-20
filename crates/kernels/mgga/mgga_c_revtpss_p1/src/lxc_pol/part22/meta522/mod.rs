//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2294;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2295;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2296;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2297;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta522<F: Float>(t1737: F, t3451: F, t1160: F, t5117: F, t1170: F, t12511: F, t12553: F, t16809: F, t16832: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t16945: F, t16998: F, t17020: F, t3454: F, t435: F, t5125: F, t3476: F, t16868: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16727: F, t16748: F, t16871: F, t16876: F, t16892: F, t16708: F, t16710: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t16927: F, t16931: F, t16933: F, t12252: F, t12261: F, t12263: F, t12265: F, t12459: F, t12460: F, t16731: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16887: F, t16890: F, t16895: F, t16898: F, t16901: F, t16904: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17023, t17026, t17029) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2294::<F>(t1737, t3451, t1160, t5117, t1170, t12511, t12553, t16809, t16832, t16837, t16839, t16842, t16844, t16846, t16945, t16998, t17020, t3454, t435, t5125);
        let (t17032, t17050, t17052, t17061) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2295::<F>(t1737, t3476, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17066, t17075, t17083) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2296::<F>(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17085 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2297::<F>(t12252, t12261, t12263, t12265, t12459, t12460, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17061, t17066, t17083);
    (t17023, t17026, t17029, t17032, t17050, t17052, t17066, t17075, t17085)
}
