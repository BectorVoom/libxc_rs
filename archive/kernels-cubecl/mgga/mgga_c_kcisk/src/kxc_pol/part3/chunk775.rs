//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 775/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk775<F: Float>(t11865: F, t11921: F, t716: F, t736: F, t11226: F, t740: F, t11228: F, t747: F, t746: F, t1941: F, t5274: F, t11764: F, t11767: F, t11770: F, t11772: F, t11778: F, t11782: F, t11784: F, t11787: F, t11790: F, t11792: F, t11796: F, t11800: F, t11804: F, t11810: F, t11813: F, sigma2: F) -> (F, F, F, F) {
    let t11922 = t11865 + t11921;
    let t11923 = t11922 * t716;
    let t11924 = t11923 * sigma2;
    let t11925 = t11924 * t736;
    let t11927 = t11226 * t740;
    let t11928 = t747 * t11228;
    let t11929 = t746 * t11928;
    let t11930 = t11927 * t11929;
    let t11932 = t5274 * t1941;
    let t11934 = -t11764 / F::cast_from(4.0_f64) - t11767 / F::cast_from(24.0_f64) - F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t11770 - t11772 / F::cast_from(24.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t11778 - t11782 / F::cast_from(192.0_f64) + F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t11784 + t11787 / F::cast_from(64.0_f64) + t11790 / F::cast_from(192.0_f64) - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t11792 + t11796 / F::cast_from(54.0_f64) - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t11800 - t11804 / F::cast_from(16.0_f64) + t11810 / F::cast_from(864.0_f64) - t11813 / F::cast_from(192.0_f64) + t11925 / F::cast_from(16.0_f64) + F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t11930 + t11932 / F::cast_from(8.0_f64);
    (t11925, t11930, t11932, t11934)
}
