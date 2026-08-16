//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1172;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1173;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1174;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1175;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1176;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta258<F: Float>(t1030: F, t1940: F, t354: F, t1036: F, t1942: F, t1039: F, t1000: F, t1025: F, t1046: F, t1935: F, t1937: F, t350: F, t378: F, t6712: F, t6716: F, t6717: F, t6723: F, t6728: F, t6730: F, t6735: F, t6742: F, t6747: F, t6750: F, t6755: F, t349: F, t1946: F, t225: F, t1065: F, t1955: F, t3174: F, t1949: F, t968: F, t1920: F, t6688: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6758, t6759, t6763, t6764) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1172::<F>(t1030, t1940, t354, t1036, t1942, t1039);
        let t6765 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1173::<F>(t354, t6764);
        let t6768 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1174::<F>(t1000, t1025, t1046, t1935, t1937, t350, t378, t6712, t6716, t6717, t6723, t6728, t6730, t6735, t6742, t6747, t6750, t6755, t6759, t6763, t6765);
        let (t6769, t6771) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1175::<F>(t349, t6768, t1946, t225);
        let t6776 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1176::<F>(t1065, t1955, t3174);
        let (t6781, t6783, t6784) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1177::<F>(t1949, t968, t1920, t225, t6688);
    (t6758, t6759, t6763, t6764, t6765, t6768, t6769, t6771, t6776, t6781, t6783, t6784)
}
