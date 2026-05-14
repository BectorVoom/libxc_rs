//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1260/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1260<F: Float>(t1045: F, t12508: F, t158: F, t1784: F, t1790: F, t1791: F, t1812: F, t184: F, t188: F, t19873: F, t24015: F, t24753: F, t2678: F, t2702: F, t3467: F, t3487: F, t5418: F, t622: F, t633: F, t634: F, t7173: F, t7174: F, t9020: F, t9033: F, t9037: F, t9042: F, t9095: F, t9096: F) -> (F,) {
    let t24757 = -0.39512695097613069591e1 * t184 * t9033 * t1812 - 0.15805078039045227836e2 * t19873 * t12508 * t2702 - 0.13170898365871023197e1 * t9020 * t634 + 0.26341796731742046394e1 * t184 * t1790 * t9095 * t633 + 0.13170898365871023197e1 * t184 * t9042 * t1812 - 0.13170898365871023197e1 * t622 * t9096 - 0.13170898365871023197e1 * t1045 * t7174 + 0.26341796731742046394e1 * t184 * t1790 * t24015 + 0.13170898365871023197e1 * t1784 * t3467 + 0.26341796731742046394e1 * t184 * t2678 * t7173 - 0.39512695097613069591e1 * t184 * t5418 * t3487 * t1791 + 0.52683593463484092788e1 * t622 * t9037 + 0.65854491829355115987e0 * t24753 * t158 * t188;
    (t24757,)
}
