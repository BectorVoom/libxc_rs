//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1107/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1107<F: Float>(t1877: F, t2057: F, t2219: F, t1408: F, t24191: F, t24339: F, t25: F, t25015: F, t25021: F, t25024: F, t25028: F, t2522: F, t25366: F, t25375: F, t25377: F, t25381: F, t25385: F, t25392: F, t26563: F, t26740: F, t26744: F, t26756: F, t606: F, t6542: F, t6671: F, t7110: F, t7114: F, t7475: F, t7545: F, t7845: F) -> (F, F) {
    let t26774 = t1877 * t2057 * t2219;
    let t26775 = F::cast_from(3.0_f64) * t26563 * t25015 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7110 * t7475 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24191 * t25021 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t25024 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t25028 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7845 * t6542 + t1877 * t26740 * t25 / F::cast_from(2.0_f64) - t1877 * t26744 * t6671 / F::cast_from(2.0_f64) + t1877 * t7845 * t606 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24191 * t25366 - t1877 * t24339 * t7545 / F::cast_from(2.0_f64) + t26756 * t25375 - t1877 * t7114 * t25377 / F::cast_from(2.0_f64) - t1877 * t7114 * t25381 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t25385 + t1877 * t7110 * t1408 / F::cast_from(2.0_f64) - t1877 * t7114 * t25392 / F::cast_from(2.0_f64) + t26774;
    (t26774, t26775)
}
