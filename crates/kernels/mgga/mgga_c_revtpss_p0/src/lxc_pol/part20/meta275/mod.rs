//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1130;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta275<F: Float>(t271: F, t2857: F, t11144: F, t10356: F, t1012: F, t11150: F, t3252: F, t11156: F, t4919: F, t11165: F, t4915: F, t1066: F, t11169: F, t247: F, t1011: F, t1025: F, t1063: F, t11802: F, t11806: F, t11811: F, t11814: F, t11818: F, t3177: F, t3184: F, t3188: F, t3241: F, t3248: F, t3255: F, t4837: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11821, t11823, t11824, t11828, t11829, t11836, t11839, t11845) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1130::<F>(t271, t2857, t11144, t10356, t1012, t11150, t3252, t11156, t4919, t11165, t4915, t1066, t11169, t247);
        let t11850 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1131::<F>(t1011, t1025, t1063, t11802, t11806, t11811, t11814, t11818, t11824, t11829, t11836, t11839, t11845, t3177, t3184, t3188, t3241, t3248, t3255, t4837);
    (t11821, t11823, t11824, t11828, t11829, t11836, t11839, t11845, t11850)
}
