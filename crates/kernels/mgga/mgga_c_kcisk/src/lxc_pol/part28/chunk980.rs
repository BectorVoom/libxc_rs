//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 980/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk980<F: Float>(t22784: F, t587: F, t16356: F, t6839: F, t1663: F, t8550: F, t4742: F, t2382: F, t6834: F, t4704: F, t8577: F, t10715: F, t8574: F, t4744: F, t8573: F, t6838: F) -> (F, F, F, F, F, F, F, F) {
    let t22786 = 0.62182e-1 * t22784 * t587;
    let t22788 = 0.32163648644302209644e2 * t16356 * t6839;
    let t22789 = t8550 * t1663;
    let t22791 = 6.0 * t4742 * t22789;
    let t22792 = t2382 * t6834;
    let t22794 = 4.0 * t4704 * t22792;
    let t22795 = t8577 * t1663;
    let t22797 = 0.96490945932906628932e2 * t10715 * t22795;
    let t22798 = t8574 * t1663;
    let t22800 = 2.0 * t4704 * t22798;
    let t22801 = t8573 * t4744;
    let t22802 = t22801 * t1663;
    let t22804 = 0.16081824322151104822e2 * t4742 * t22802;
    let t22805 = t6838 * t6834;
    (t22786, t22788, t22791, t22794, t22797, t22800, t22804, t22805)
}
