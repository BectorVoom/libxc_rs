//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 927/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk927<F: Float>(t17757: F, t2493: F, t17736: F, t17776: F, t9896: F, t17740: F, t17744: F, t3917: F, t17780: F, t17722: F, t18303: F, t18305: F, t18308: F, t18312: F, t18314: F, t18316: F, t18318: F, t18321: F, t18324: F, t18327: F, t18330: F, t18333: F, t18336: F, t3139: F, t462: F, t92: F) -> F {
    let t18339 = t2493 * t17757;
    let t18342 = t2493 * t17736;
    let t18345 = t9896 * t17776;
    let t18348 = t2493 * t17740;
    let t18351 = t3917 * t17744;
    let t18354 = t3917 * t17780;
    let t18357 = t2493 * t17722;
    let t18360 = F::new(2.0) / F::new(27.0) * t18303 - F::new(2.0) / F::new(9.0) * t18305 - t462 * t18308 / F::new(3.0) - t92 * t18312 + t18314 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t18316 + F::new(2.0) / F::new(3.0) * t462 * t18318 - F::new(2.0) / F::new(9.0) * t462 * t18321 + t462 * t18324 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t462 * t18327 + F::new(4.0) / F::new(3.0) * t462 * t18330 - F::new(10.0) / F::new(27.0) * t462 * t18333 + F::new(8.0) / F::new(9.0) * t3139 * t18336 + F::new(2.0) / F::new(3.0) * t462 * t18339 - F::new(4.0) / F::new(3.0) * t3139 * t18342 - F::new(2.0) / F::new(3.0) * t462 * t18345 - F::new(2.0) / F::new(3.0) * t462 * t18348 - F::new(2.0) * t462 * t18351 - F::new(8.0) / F::new(3.0) * t3139 * t18354 + t462 * t18357 / F::new(3.0);
    t18360
}
