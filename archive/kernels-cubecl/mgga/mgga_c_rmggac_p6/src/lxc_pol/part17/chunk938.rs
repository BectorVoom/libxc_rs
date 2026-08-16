//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 938/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk938<F: Float>(t7691: F, t9783: F, t1835: F, t1979: F, t1982: F, t201: F, t457: F, t2191: F, t9932: F, t39506: F, t39529: F, t39536: F, t39545: F, t39556: F, t39558: F, t45579: F, t45584: F, t45589: F, t45591: F, t45593: F, t45595: F, t45597: F, t45599: F, t45601: F) -> F {
    let t45603 = t7691 * t9783;
    let t45608 = t1835 * t457 * t201 * t1979 * t1982;
    let t45610 = t2191 * t9932;
    let t45612 = -F::cast_from(0.6818665413561335432e-1_f64) * t45579 - F::cast_from(0.1064114997332445985e-4_f64) * t45584 + F::cast_from(0.25538759935978703638e-4_f64) * t45589 - F::cast_from(0.4726e1_f64) * t45591 - F::cast_from(0.14967802127329760705e-1_f64) * t45593 - t39506 - F::cast_from(0.2993560425465952141e-1_f64) * t45595 + F::cast_from(0.19863479950205658386e-4_f64) * t45597 - F::cast_from(0.59590439850616975155e-4_f64) * t45599 - t39529 + t39536 + t39545 - F::cast_from(0.1064114997332445985e-4_f64) * t45601 - F::cast_from(0.53205749866622299248e-5_f64) * t45603 - t39556 - t39558 + F::cast_from(0.42564599893297839398e-5_f64) * t45608 + F::cast_from(0.85129199786595678796e-5_f64) * t45610;
    t45612
}
