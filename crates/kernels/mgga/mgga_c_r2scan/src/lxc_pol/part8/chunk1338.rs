//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1338/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1338<F: Float>(t10125: F, t1632: F, t551: F, t6218: F, t2551: F, t32444: F, t481: F, t9937: F, t2526: F, t2892: F, t3016: F, t10092: F, t20151: F, t2196: F, t22836: F, t23007: F, t2564: F, t2598: F, t27867: F, t28198: F, t28202: F, t28206: F, t28225: F, t28240: F, t360: F, t5109: F, t552: F, t560: F, t6493: F, t6528: F, t8820: F, t9507: F) -> (F, F, F, F, F) {
    let t32773 = t6218 * t551 * t1632 * t10125;
    let t32777 = t32444 * t2551;
    let t32787 = t9937 * t481;
    let t32792 = t2892 * t2526;
    let t32799 = t2526 * t3016;
    let t32809 = 0.13002332610081402845e0 * t27867 * t2564 + 0.6112917064160653851e0 * t28198 + 0.20803732176130244552e1 * t32773 + 0.20803732176130244552e2 * t28202 - 0.20803732176130244552e1 * t28206 + 0.7801399566048841707e1 * t23007 * t5109 * t32777 - 0.69345773920434148506e0 * t28225 + 0.2600466522016280569e1 * t20151 * t551 * t552 * t9937 * t560 + 0.15602799132097683414e2 * t22836 * t551 * t552 * t32787 - 0.7801399566048841707e1 * t6528 * t551 * t552 * t32792 + 0.15602799132097683414e1 * t6493 * t10092 + 0.15602799132097683414e1 * t2196 * t551 * t552 * t32799 + 0.69345773920434148506e0 * t28240 + 0.78013995660488417067e0 * t2598 * t360 * t8820 * t9507;
    (t32777, t32787, t32792, t32799, t32809)
}
