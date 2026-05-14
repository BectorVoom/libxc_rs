//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 816/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk816<F: Float>(t11141: F, t11352: F, t1291: F, t1603: F, t1665: F, t1669: F, t22702: F, t22705: F, t22709: F, t22711: F, t22715: F, t22719: F, t22722: F, t22726: F, t22736: F, t22738: F, t22743: F, t22747: F, t22752: F, t22756: F, t22761: F, t22762: F, t22767: F, t22775: F, t22777: F, t3066: F, t3076: F, t5538: F, t5540: F, t5577: F, t5579: F, t5580: F, t5598: F, t5599: F, t79: F, t7982: F) -> (F,) {
    let t22781 = -4.0 * t1669 * t22702 - 2.0 * t1669 * t22705 - 0.38482339615903025572e-7 * t3076 * t22709 * t22711 + 0.13519760450715832853e-3 * t7982 * t22715 - 0.46509801892875584e-1 * t1603 * t22719 - 0.23254900946437792e-1 * t1603 * t22722 + 0.89591295428265718861e-3 * t79 * t22726 - 0.44455354858818847408e-2 * t1665 * t1291 + 0.25845121844514357744e-4 * t5538 * t5540 * t11352 + 0.12255510004984495842e-5 * t22736 * t22738 * t3066 - 0.1721820212247325051e-5 * t5538 * t22743 * t11141 + 0.38306165027777777778e-1 * t5598 * t5579 * t22747 - 0.38306165027777777778e-1 * t5577 * t22752 + 4.0 * t1669 * t22756 - 0.11491849508333333333e0 * t22761 * t5579 * t22762 - 0.20429954681481481482e0 * t5598 * t22767 * t5599 + 0.20429954681481481482e0 * t5577 * t22767 * t5580 - 0.25537443351851851852e-1 * t22775 - 0.27568129967481981593e-3 * t5538 * t22777 * t3066;
    (t22781,)
}
