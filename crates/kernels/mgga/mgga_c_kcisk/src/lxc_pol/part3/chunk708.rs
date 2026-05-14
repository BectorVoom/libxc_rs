//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 708/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk708<F: Float>(t11865: F, t11921: F, t716: F, t736: F, t11226: F, t740: F, t11228: F, t747: F, t746: F, t1941: F, t5274: F, t11764: F, t11767: F, t11770: F, t11772: F, t11778: F, t11782: F, t11784: F, t11787: F, t11790: F, t11792: F, t11796: F, t11800: F, t11804: F, t11810: F, t11813: F, sigma2: F) -> (F, F, F, F) {
    let t11922 = t11865 + t11921;
    let t11923 = t11922 * t716;
    let t11924 = t11923 * sigma2;
    let t11925 = t11924 * t736;
    let t11927 = t11226 * t740;
    let t11928 = t747 * t11228;
    let t11929 = t746 * t11928;
    let t11930 = t11927 * t11929;
    let t11932 = t5274 * t1941;
    let t11934 = -t11764 / 4.0 - t11767 / 24.0 - 3.0 / 128.0 * t11770 - t11772 / 24.0 - 3.0 / 8.0 * t11778 - t11782 / 192.0 + 3.0 / 256.0 * t11784 + t11787 / 64.0 + t11790 / 192.0 - 3.0 / 16.0 * t11792 + t11796 / 54.0 - 3.0 / 16.0 * t11800 - t11804 / 16.0 + t11810 / 864.0 - t11813 / 192.0 + t11925 / 16.0 + 3.0 / 128.0 * t11930 + t11932 / 8.0;
    (t11925, t11930, t11932, t11934)
}
