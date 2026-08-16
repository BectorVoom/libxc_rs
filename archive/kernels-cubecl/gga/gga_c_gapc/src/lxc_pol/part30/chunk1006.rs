//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1006/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1006<F: Float>(t12449: F, t12463: F, t3879: F, t883: F, t3883: F, t972: F, t1125: F, t3622: F, t3897: F, t11728: F, t11731: F, t11734: F, t11737: F, t11739: F, t11743: F, t11746: F, t11750: F, t11756: F, t11762: F, t11765: F, t11767: F, t11770: F, t11773: F, t11776: F, t11779: F, t11782: F, t11785: F, t11787: F) -> (F, F, F, F, F, F) {
    let t12464 = t12449 + t12463;
    let t12466 = t3879 * t883;
    let t12476 = t3883 * t972;
    let t12479 = t1125 * t3622;
    let t12483 = t3897 * t972;
    let t12505 = F::cast_from(0.43440462632258606772e-4_f64) * t11728 + F::cast_from(0.43440462632258606772e-4_f64) * t11731 - F::cast_from(0.4637672555408563478e-4_f64) * t11734 + F::cast_from(0.24581606547037760419e-7_f64) * t11737 + F::cast_from(0.3243554543208642639e-2_f64) * t11739 - F::cast_from(0.43440462632258606772e-4_f64) * t11743 - F::cast_from(0.11372686522837130914e-5_f64) * t11746 + F::cast_from(0.4637672555408563478e-4_f64) * t11750 - F::cast_from(0.505954834707648426e-7_f64) * t11756 + F::cast_from(0.2445773654513888889e-4_f64) * t11762 - F::cast_from(0.69504740211613770836e-3_f64) * t11765 - F::cast_from(0.69504740211613770836e-3_f64) * t11767 + F::cast_from(0.2845640240200497334e-7_f64) * t11770 + F::cast_from(0.25301106770833333335e-5_f64) * t11773 + F::cast_from(0.25301106770833333335e-5_f64) * t11776 + F::cast_from(0.33816362383187442027e-5_f64) * t11779 + F::cast_from(0.49520679385353736436e-5_f64) * t11782 + F::cast_from(0.21102562238076876322e-7_f64) * t11785 + F::cast_from(0.21102562238076876322e-7_f64) * t11787;
    (t12464, t12466, t12476, t12479, t12483, t12505)
}
