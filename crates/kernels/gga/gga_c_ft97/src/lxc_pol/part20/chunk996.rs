//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 996/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk996<F: Float>(t294: F, t43194: F, t1526: F, t4037: F, t9483: F, t18972: F, t52679: F, t4052: F, t2252: F, t342: F, t5202: F, t18982: F, t630: F, t42262: F, t5198: F, t13315: F, t13616: F, t14624: F, t14628: F, t14648: F, t14653: F, t14660: F, t14664: F, t14889: F, t14906: F, t14911: F, t14914: F, t15567: F, t2248: F, t231: F, t2320: F, t343: F, t3806: F, t668: F, t703: F) -> (F,) {
    let t72944 = t43194 * t294;
    let t72950 = t1526 * t9483 * t4037 / 18.0;
    let t72952 = t1526 * t52679 * t18972;
    let t72962 = t1526 * t9483 * t4052 / 18.0;
    let t72977 = t342 * t2252 * t5202;
    let t72981 = t342 * t630 * t18982 / 6.0;
    let t72992 = t1526 * t42262 * t5198;
    let t72994 = -7.0 / 27.0 * t15567 * t72944 * t13315 - t72950 - 7.0 / 18.0 * t72952 - t1526 * t2320 * t14624 / 12.0 + t1526 * t13616 * t14628 / 3.0 - t72962 - t1526 * t2320 * t14660 / 6.0 - t1526 * t2320 * t14664 / 12.0 - t1526 * t3806 * t14653 / 9.0 - t342 * t343 * t231 * t14889 / 4.0 + t72977 / 18.0 - t72981 + 2.0 * t14911 + t14906 + t1526 * t2248 * t703 * t294 * t668 / 6.0 + t14914 + t1526 * t2320 * t14648 / 6.0 + t72992 / 54.0;
    (t72994,)
}
