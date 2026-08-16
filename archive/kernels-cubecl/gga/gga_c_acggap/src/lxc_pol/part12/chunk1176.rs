//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1176/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1176<F: Float>(t34609: F, t34618: F, t34620: F, t34626: F, t34632: F, t34650: F, t30659: F, t34611: F, t34614: F, t34616: F, t34622: F, t34630: F, t34636: F, t34638: F, t34640: F, t34644: F, t34647: F) -> F {
    let t37175 = F::cast_from(11.0_f64) / F::cast_from(96.0_f64) * t34609;
    let t37179 = F::cast_from(0.2264262644851498949e-1_f64) * t34618;
    let t37180 = F::cast_from(0.37737710747524982482e-2_f64) * t34620;
    let t37182 = F::cast_from(0.18868855373762491241e-2_f64) * t34626;
    let t37184 = F::cast_from(0.37737710747524982482e-1_f64) * t34632;
    let t37190 = F::cast_from(0.22921875e-1_f64) * t34650;
    let t37192 = -t37175 - F::cast_from(0.94344276868812456208e-2_f64) * t34611 + F::cast_from(0.42874018118069736972e-2_f64) * t34614 - F::cast_from(0.37737710747524982483e-1_f64) * t34616 - t37179 + t37180 - F::cast_from(0.75475421495049964966e-2_f64) * t34622 - t37182 + F::cast_from(0.12862205435420921092e-1_f64) * t34630 - t37184 - F::cast_from(0.94344276868812456207e-3_f64) * t34636 + F::cast_from(0.31448092289604152069e-3_f64) * t34638 + F::cast_from(0.56606566121287473722e-1_f64) * t34640 - F::cast_from(0.94344276868812456204e-2_f64) * t34644 + F::cast_from(0.1528125e-1_f64) * t34647 + t37190 + F::cast_from(0.51448821741683684367e-2_f64) * t30659;
    t37192
}
