//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta795 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2616;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta795<F: Float>(t10760: F, t18409: F, t9794: F, t10777: F, t10779: F, t5984: F, t837: F, t18414: F, t40799: F, t18418: F, t18392: F, t236: F, t807: F, t854: F, t18643: F, t40731: F, t10786: F, t14931: F, t61956: F, t10811: F, t18647: F, t18511: F, t40864: F, t10905: F, t18515: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t61981, t61985, t62012, t62015, t62021) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2616::<F>(t10760, t18409, t9794, t10777, t10779, t5984, t837, t18414, t40799, t18418, t18392, t236, t807, t854);
        let (t62029, t62033, t62045, t62056, t62058) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2617::<F>(t18643, t40731, t10779, t10786, t14931, t61956, t10811, t18647, t18511, t40864, t10905, t18515);
    (t61981, t61985, t62012, t62015, t62021, t62029, t62033, t62045, t62056, t62058)
}
