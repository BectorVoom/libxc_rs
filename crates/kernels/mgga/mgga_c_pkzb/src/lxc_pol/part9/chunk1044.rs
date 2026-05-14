//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1044/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1044<F: Float>(t12: F, t2620: F, t5322: F, t1532: F, t2557: F, t49: F, t4865: F, t7046: F, t4868: F, t1429: F, t1643: F, t1646: F, t439: F, t82: F, t2543: F, t500: F, t16232: F, t1642: F, t2540: F, t5093: F, t5094: F, t5100: F, t6767: F, t6770: F, t8: F, t87: F, t972: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t19620 = t2620 * t5322;
    let t19621 = 0.56968947174242584612e-3 * t19620;
    let t19623 = t2557 * t49 * t1532;
    let t19624 = 0.32530743900905219526e-1 * t19623;
    let t19625 = t7046 * t4865;
    let t19626 = 0.16265371950452609763e-1 * t19625;
    let t19627 = t7046 * t4868;
    let t19628 = 0.48159733137676571078e0 * t19627;
    let t19633 = t1429 * t1643;
    let t19636 = t439 * t1646;
    let t19642 = t82 * t439;
    let t19645 = t1429 * t1646;
    let t19653 = 32.0 * t2543 * t500;
    let t19655 = piecewise3(t84, 0.0, 40.0 / 81.0 * t16232 * t972 * t5094 - 16.0 / 9.0 * t5093 * t8 * t19633 - 8.0 / 9.0 * t6767 * t19636 + 8.0 / 3.0 * t1642 * t1429 * t439 - 8.0 * t6770 * t19642 + 8.0 / 3.0 * t6770 * t19645 + 4.0 / 9.0 * t2540 * t5100 - 16.0 * t87 * t82 + t19653);
    (t19621, t19624, t19626, t19628, t19633, t19636, t19642, t19645, t19655)
}
