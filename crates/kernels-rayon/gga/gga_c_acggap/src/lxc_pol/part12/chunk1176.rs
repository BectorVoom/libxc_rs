//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1176/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1176(t34609: f64, t34618: f64, t34620: f64, t34626: f64, t34632: f64, t34650: f64, t30659: f64, t34611: f64, t34614: f64, t34616: f64, t34622: f64, t34630: f64, t34636: f64, t34638: f64, t34640: f64, t34644: f64, t34647: f64) -> f64 {
    let t37175 = 11.0_f64 / 96.0_f64 * t34609;
    let t37179 = 0.2264262644851498949e-1_f64 * t34618;
    let t37180 = 0.37737710747524982482e-2_f64 * t34620;
    let t37182 = 0.18868855373762491241e-2_f64 * t34626;
    let t37184 = 0.37737710747524982482e-1_f64 * t34632;
    let t37190 = 0.22921875e-1_f64 * t34650;
    let t37192 = -t37175 - 0.94344276868812456208e-2_f64 * t34611 + 0.42874018118069736972e-2_f64 * t34614 - 0.37737710747524982483e-1_f64 * t34616 - t37179 + t37180 - 0.75475421495049964966e-2_f64 * t34622 - t37182 + 0.12862205435420921092e-1_f64 * t34630 - t37184 - 0.94344276868812456207e-3_f64 * t34636 + 0.31448092289604152069e-3_f64 * t34638 + 0.56606566121287473722e-1_f64 * t34640 - 0.94344276868812456204e-2_f64 * t34644 + 0.1528125e-1_f64 * t34647 + t37190 + 0.51448821741683684367e-2_f64 * t30659;
    t37192
}
