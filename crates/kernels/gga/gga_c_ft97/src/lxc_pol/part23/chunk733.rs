//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 733/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk733<F: Float>(t10864: F, t18934: F, t505: F, t5468: F, t668: F, t2923: F, t10839: F, t12143: F, t18877: F, t18880: F, t18884: F, t18887: F, t18889: F, t18893: F, t18896: F, t18900: F, t18902: F, t18905: F, t18908: F, t18911: F, t18914: F, t18919: F, t18923: F, t18928: F, t18931: F, t2265: F, t631: F) -> (F,) {
    let t18936 = t10864 * t18934 * t505;
    let t18938 = t5468 * t668;
    let t18939 = t18938 * t505;
    let t18940 = t2923 * t18939;
    let t18943 = 4.0 / 9.0 * t18877 + 4.0 / 3.0 * t12143 * t18880 + 2.0 / 3.0 * t2265 * t18884 + t2265 * t18887 + 4.0 / 3.0 * t12143 * t18889 - t2265 * t18893 / 3.0 - t2265 * t18896 / 3.0 + 5.0 / 9.0 * t10839 + 2.0 / 9.0 * t18900 - t18902 / 9.0 - t2265 * t18905 / 9.0 - t2265 * t18908 / 3.0 + 2.0 / 27.0 * t2265 * t18911 - 2.0 / 9.0 * t12143 * t18914 - 3.0 / 2.0 * t631 * t18919 + t631 * t18923 / 6.0 + 6.0 * t631 * t18928 + t2265 * t18931 / 18.0 + t2265 * t18936 - t2265 * t18940 / 3.0;
    (t18943,)
}
