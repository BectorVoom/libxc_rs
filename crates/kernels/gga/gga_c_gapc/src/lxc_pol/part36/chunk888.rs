//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 888/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk888<F: Float>(t11792: F, t11796: F, t11800: F, t11806: F, t11809: F, t11811: F, t11816: F, t11818: F, t11820: F, t11823: F, t11826: F, t11829: F, t11832: F, t11838: F, t11841: F, t11843: F, t11845: F, t11851: F, t11855: F, t11858: F) -> (F,) {
    let t12526 = 0.69504740211613770836e-3 * t11792 - 0.10298285674687440379e-4 * t11796 - 0.43440462632258606772e-4 * t11800 + 0.49163213094075520838e-8 * t11806 - 0.70341874126922921073e-8 * t11809 + 0.66295654499063700024e-7 * t11811 + 0.24581606547037760419e-8 * t11816 - 0.64085799349094910023e-6 * t11818 + 0.22509399720615334744e-6 * t11820 - 0.6070699179094394313e-6 * t11823 + 0.10793703140429833089e-5 * t11826 - 0.24581606547037760419e-8 * t11829 + 0.16387737698025173613e-8 * t11832 - 0.22098551499687900008e-7 * t11838 - 0.11594181388521408695e-4 * t11841 + 0.2318836277704281739e-4 * t11843 - 0.2318836277704281739e-4 * t11845 + 0.57920616843011475696e-5 * t11851 - 0.50680539737635041234e-3 * t11855 - 0.25301106770833333335e-5 * t11858;
    (t12526,)
}
