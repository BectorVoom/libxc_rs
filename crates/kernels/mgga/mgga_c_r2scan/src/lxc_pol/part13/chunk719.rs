//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 719/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk719<F: Float>(t1938: F, t5636: F, t1956: F, t206: F, t1923: F, t689: F, t1399: F, t1810: F, t1814: F, t1827: F, t1831: F, t1838: F, t390: F, t5612: F, t5614: F, t5629: F, t5633: F, t741: F, t750: F) -> F {
    let t5637 = t5636 * t1938;
    let t5640 = t1956 * t206;
    let t5641 = t689 * t1923;
    let t5642 = t5640 * t5641;
    let t5647 = t5612 - t5614 + F::new(0.32530743900905219526e-1) * t390 * t1831 + F::new(0.28895839882605942646e1) * t390 * t1838 + F::new(0.65061487801810439052e-1) * t390 * t1827 - F::new(0.97592231702715658578e-1) * t390 * t1810 - F::new(0.43374325201206959369e-1) * t1399 * t741 + F::new(0.64212977516902094772e0) * t1399 * t750 - F::new(0.10628661134652298293e3) * t390 * t5629 - F::new(0.16522625736956710527e1) * t390 * t5633 - F::new(0.33045251473913421054e1) * t390 * t5637 + F::new(0.99135754421740263165e1) * t390 * t5642 - F::new(0.48159733137676571079e0) * t390 * t1814;
    t5647
}
