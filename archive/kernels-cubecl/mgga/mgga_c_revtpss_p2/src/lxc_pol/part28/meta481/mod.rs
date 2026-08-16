//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1825;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1826;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1827;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1828;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1829;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1830;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta481<F: Float>(t25516: F, t3114: F, t1068: F, t25543: F, t25551: F, t25554: F, t25557: F, t25560: F, t25561: F, t25564: F, t25566: F, t25569: F, t25577: F, t3101: F, t3120: F, t3177: F, t3184: F, t3238: F, t3248: F, t3255: F, t375: F, t7111: F, t7132: F, t25542: F, t1984: F, t359: F, t3057: F, t7143: F, t7145: F, t7146: F, t999: F, t1096: F, t7152: F, t7160: F, t1035: F, t8515: F, t1983: F, t1043: F, t7161: F, t1089: F, t378: F, t7150: F, t8521: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t25580 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1825::<F>(t25516, t3114);
        let t25585 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1826::<F>(t1068, t25543, t25551, t25554, t25557, t25560, t25561, t25564, t25566, t25569, t25577, t25580, t3101, t3120, t3177, t3184, t3238, t3248, t3255, t375, t7111, t7132);
        let t25586 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1827::<F>(t25542, t25585);
        let (t25588, t25591) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1828::<F>(t1984, t25586, t359, t3057, t7143);
        let (t25593, t25597, t25601, t25604) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1829::<F>(t7145, t7146, t999, t1096, t7152, t7160, t1035, t8515);
        let t25605 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1830::<F>(t1983, t25604);
        let (t25607, t25610, t25611) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1831::<F>(t1043, t7161, t1089, t378, t7150, t8521);
    (t25580, t25586, t25588, t25591, t25593, t25597, t25601, t25604, t25605, t25607, t25610, t25611)
}
