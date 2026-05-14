//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1298/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1298<F: Float>(t104965: F, t23825: F, t23842: F, t104932: F, t104941: F, t104945: F, t104949: F, t104953: F, t104955: F, t104956: F, t23774: F, t23832: F, t23839: F, t23847: F, t3380: F, t40234: F, t5579: F, t5790: F, t61786: F, t72: F, t8812: F, t8838: F, t94578: F, t94582: F, t94600: F) -> (F,) {
    let t104967 = 0.1611184118048991131e0 * t23825 * t104965;
    let t104969 = 0.1611184118048991131e0 * t23842 * t104965;
    let t104970 = 0.48327307107230638237e1 * t23847 * t104932 - 0.10947790369858991997e1 * t8812 * t5790 * t3380 - 0.59269334304526748975e-1 * t94578 - 0.16299066933744855968e0 * t94582 - t94600 - 0.13592055123908617004e1 * t40234 * t104941 - 0.48327307107230638237e1 * t8838 * t104945 + 0.66678001092592592594e-1 * t104949 - t104953 + t104955 - 0.28195722065857344792e1 * t23832 * t104956 - 0.30005100491666666667e0 * t23774 * t5579 * t72 * t61786 + 0.28195722065857344792e1 * t23839 * t104956 + t104967 - t104969;
    (t104970,)
}
