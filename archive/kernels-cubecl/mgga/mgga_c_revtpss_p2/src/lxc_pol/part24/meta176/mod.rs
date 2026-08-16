//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta176 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk872;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk873;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk874;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta176<F: Float>(t1868: F, t5532: F, t3854: F, t3859: F, t3862: F, t3865: F, t3867: F, t3871: F, t3873: F, t4027: F, t4035: F, t4037: F, t4042: F, t4139: F, t6827: F, t6828: F, t6929: F, t118: F, t1502: F, t1519: F, t1843: F, t1847: F, t1911: F, t4248: F, t508: F, t511: F, t569: F, t5877: F, t5884: F, t5887: F, t5921: F, t651: F, t6765: F, t6773: F, t3: F, t116: F, t5883: F, t117: F, t5920: F, t1916: F, t1918: F, t572: F, t573: F, t159: F, t793: F, param_d: F, t1493: F, t76: F, t1518: F, t94: F, t93: F, t587: F, t65: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t6933 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk872::<F>(t1868, t5532, t3854, t3859, t3862, t3865, t3867, t3871, t3873, t4027, t4035, t4037, t4042, t4139, t6827, t6828);
        let (t6934, t6936) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk873::<F>(t6929, t6933, t118, t1502, t1519, t1843, t1847, t1911, t4248, t508, t511, t569, t5877, t5884, t5887, t5921, t651, t6765, t6773);
        let (t6937, t6941, t6945, t6948, t6951, t7021) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk874::<F>(t3, t6936, t116, t5883, t117, t5920, t1916, t1918, t572, t573, t159, t793, param_d);
        let (t7719, t7732, t7889, t8779) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk875::<F>(t1493, t76, t1518, t94, t93, t587, t65);
    (t6934, t6936, t6937, t6941, t6945, t6948, t6951, t7021, t7719, t7732, t7889, t8779)
}
