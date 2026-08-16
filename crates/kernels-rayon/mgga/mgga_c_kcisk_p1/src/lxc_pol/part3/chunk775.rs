//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 775/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk775(t11865: f64, t11921: f64, t716: f64, t736: f64, t11226: f64, t740: f64, t11228: f64, t747: f64, t746: f64, t1941: f64, t5274: f64, t11764: f64, t11767: f64, t11770: f64, t11772: f64, t11778: f64, t11782: f64, t11784: f64, t11787: f64, t11790: f64, t11792: f64, t11796: f64, t11800: f64, t11804: f64, t11810: f64, t11813: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t11922 = t11865 + t11921;
    let t11923 = t11922 * t716;
    let t11924 = t11923 * sigma2;
    let t11925 = t11924 * t736;
    let t11927 = t11226 * t740;
    let t11928 = t747 * t11228;
    let t11929 = t746 * t11928;
    let t11930 = t11927 * t11929;
    let t11932 = t5274 * t1941;
    let t11934 = -t11764 / 4.0_f64 - t11767 / 24.0_f64 - 3.0_f64 / 128.0_f64 * t11770 - t11772 / 24.0_f64 - 3.0_f64 / 8.0_f64 * t11778 - t11782 / 192.0_f64 + 3.0_f64 / 256.0_f64 * t11784 + t11787 / 64.0_f64 + t11790 / 192.0_f64 - 3.0_f64 / 16.0_f64 * t11792 + t11796 / 54.0_f64 - 3.0_f64 / 16.0_f64 * t11800 - t11804 / 16.0_f64 + t11810 / 864.0_f64 - t11813 / 192.0_f64 + t11925 / 16.0_f64 + 3.0_f64 / 128.0_f64 * t11930 + t11932 / 8.0_f64;
    (t11925, t11930, t11932, t11934)
}
