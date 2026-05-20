//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 841/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk841<F: Float>(t124: F, t836: F, t10779: F, t2749: F, t10777: F, t125: F, t2722: F, t2723: F, t775: F, t2747: F, t10730: F, t10734: F, t10737: F, t10742: F, t10746: F, t10749: F, t10752: F, t10756: F, t10758: F, t10762: F, t10766: F, t10773: F, t2730: F, t2745: F, t4362: F, t851: F) -> (F, F, F, F) {
    let t10780 = t124 * t836;
    let t10782 = t10779 * t10780 * t2749;
    let t10783 = t10777 * t10782;
    let t10785 = t125 * t2722;
    let t10786 = t2723 * t775;
    let t10788 = t2747 * t10785 * t10786;
    let t10791 = -F::cast_from(0.42874018118069736972e-4_f64) * t10730 + F::cast_from(0.21437009059034868486e-4_f64) * t10734 - F::cast_from(0.85748036236139473944e-3_f64) * t851 * t10737 - F::cast_from(0.15246000842785598468e-3_f64) * t10742 + F::cast_from(0.76230004213927992336e-5_f64) * t10746 - F::cast_from(0.5421477899694558815e-4_f64) * t10749 + F::new(3.0) / F::new(16.0) * t2730 * t10752 - t10756 - t10758 - F::cast_from(0.13553694749236397037e-4_f64) * t10762 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t10766 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t10773 + F::cast_from(0.30492001685571196935e-3_f64) * t10783 - F::cast_from(0.51448821741683684367e-2_f64) * t4362 * t10788;
    (t10782, t10785, t10788, t10791)
}
