//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1209/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1209<F: Float>(t11597: F, t3008: F, t3060: F, t1030: F, t11591: F, t144: F, t1461: F, t8709: F, t11601: F, t9288: F, t34905: F, t34907: F, t34909: F, t34911: F, t34914: F, t34918: F, t34921: F, t34926: F) -> F {
    let t34929 = t3060 * t11597 * t3008;
    let t34934 = t1030 * t1461 * t8709 * t144 * t11591;
    let t34936 = t11601 * t9288;
    let t34938 = -F::new(0.24583187891642252608e-8) * t34905 + F::new(0.32777583855523003478e-8) * t34907 - F::new(0.8433973524305555556e-6) * t34909 + F::new(0.73797268337673611116e-6) * t34911 + F::new(0.73797268337673611116e-6) * t34914 + F::new(0.4423264264475966605e-6) * t34918 + F::new(0.22467583330805503619e-6) * t34921 - F::new(0.11666996708622685185e-3) * t34926 + F::new(0.13506074236995523433e-5) * t34929 - F::new(0.10957550886745307093e-6) * t34934 + F::new(0.67530371184977617164e-6) * t34936;
    t34938
}
