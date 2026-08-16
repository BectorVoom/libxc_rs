//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1078/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1078<F: Float>(t30721: F, t30725: F, t30729: F, t34743: F, t34746: F, t34747: F, t34749: F, t34751: F, t34754: F, t34757: F, t34762: F, t34767: F, t34769: F, t34771: F, t34775: F, t34779: F, t34783: F, t34788: F) -> F {
    let t34790 = -t34743 - F::cast_from(0.18868855373762491241e-2_f64) * t30721 - t34746 - F::cast_from(0.68598428988911579156e-2_f64) * t34747 - F::cast_from(0.34299214494455789578e-2_f64) * t34749 + F::cast_from(0.17149607247227894789e-2_f64) * t34751 + t34754 + F::cast_from(0.64311027177104605458e-3_f64) * t34757 + F::cast_from(0.31448092289604152068e-2_f64) * t30725 + t30729 + F::cast_from(0.31448092289604152068e-3_f64) * t34762 - F::cast_from(0.41930789719472202758e-3_f64) * t34767 - F::cast_from(0.85748036236139473944e-3_f64) * t34769 - F::cast_from(0.41930789719472202758e-3_f64) * t34771 - F::cast_from(0.31448092289604152068e-3_f64) * t34775 - F::cast_from(0.62896184579208304136e-3_f64) * t34779 - F::cast_from(0.41930789719472202758e-3_f64) * t34783 - F::cast_from(0.31448092289604152068e-3_f64) * t34788;
    t34790
}
