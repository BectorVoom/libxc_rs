//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 861/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk861<F: Float>(t12630: F, t196: F, t852: F, t989: F, t816: F, t179: F, t2925: F, t15: F, t197: F, t2861: F, t183: F, t3: F, t213: F, t1011: F, t1018: F, t12454: F, t12460: F, t12462: F, t12500: F, t12503: F, t12505: F, t139: F, t172: F, t175: F, t198: F, t3194: F, t3203: F, t3209: F, t3213: F, t3220: F) -> (F,) {
    let t12631 = t12630 * t196;
    let t12636 = t852 * t989;
    let t12637 = t12636 * t816;
    let t12640 = t2925 * t179;
    let t12641 = t12640 * t15;
    let t12644 = t197 * t2861;
    let t12645 = t183 * t3;
    let t12646 = t12645 * t213;
    let t12649 = 0.74295e-1 * t12454 * t3209 + 0.4953e-1 * t3194 * t3213 - 0.619125e-2 * t12460 * t12462 - 0.619125e-2 * t197 * t12500 + 0.371475e-1 * t12503 * t12505 - 0.23583209876543209876e-1 * t139 * t172 * t175 + 0.619125e-2 * t12631 * t198 - 0.1857375e-1 * t1011 * t3220 + 0.619125e-2 * t12637 * t3203 - 0.371475e-1 * t12641 * t1018 + 0.41275e-2 * t12644 * t12646;
    (t12649,)
}
