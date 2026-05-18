//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1198/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1198<F: Float>(t1861: F, t667: F, t7360: F, t1867: F, t7365: F, t2754: F, t5540: F, t7370: F, t2765: F, t20759: F, t20762: F, t20765: F, t20769: F, t20773: F, t20777: F, t20781: F, t20789: F, t20791: F, t20794: F, t20797: F, t20800: F, t20803: F, t20806: F) -> (F, F, F, F, F, F) {
    let t20809 = t1861 * t7360 * t667;
    let t20811 = t7365 * t1867;
    let t20813 = t2754 * t5540;
    let t20815 = t7370 * t1867;
    let t20817 = t2765 * t5540;
    let t20819 = -F::new(0.49671e0) * t20759 - F::new(0.99342e0) * t20762 - F::new(0.49671e0) * t20765 + F::new(0.248355e0) * t20769 + F::new(0.745065e0) * t20773 + F::new(0.745065e0) * t20777 + F::new(0.248355e0) * t20781 + F::new(0.16504875e0) * t20789 + F::new(0.258925e1) * t20791 + F::new(0.58258125e1) * t20794 - F::new(0.1237865625e0) * t20797 - F::new(0.485484375e1) * t20800 + F::new(0.6189328125e-1) * t20803 + F::new(0.247573125e0) * t20806 - F::new(0.3883875e1) * t20809 - F::new(0.3883875e1) * t20811 - F::new(0.1294625e1) * t20813 + F::new(0.247573125e0) * t20815 + F::new(0.82524375e-1) * t20817;
    (t20809, t20811, t20813, t20815, t20817, t20819)
}
