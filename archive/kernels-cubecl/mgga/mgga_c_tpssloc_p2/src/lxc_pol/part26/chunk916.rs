//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 916/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk916<F: Float>(t2885: F, t919: F, t10717: F, t10720: F, t10724: F, t10729: F, t10733: F, t10734: F, t10739: F, t10740: F, t10744: F, t10747: F, t10750: F, t10753: F, t10756: F, t10757: F, t10760: F, t2856: F, t2861: F, t2863: F, t2881: F, t2886: F, t2889: F, t2905: F, t2907: F, t2930: F, t933: F, t943: F) -> F {
    let t10765 = t919 * t2885;
    let t10768 = F::cast_from(0.96491876992155210402e2_f64) * t2886 * t10717 - F::cast_from(0.35089341735807877242e1_f64) * t2905 * t10720 + F::cast_from(0.51947577317044391277e2_f64) * t2930 * t10724 + t10729 - t10733 - F::cast_from(6.0_f64) * t2861 * t10734 - t10739 - F::cast_from(6.0_f64) * t10740 * t2863 + F::cast_from(6.0_f64) * t2886 * t10744 - F::cast_from(0.35089341735807877242e1_f64) * t10747 * t2907 + F::cast_from(0.35089341735807877242e1_f64) * t2930 * t10750 + F::cast_from(0.5848223622634646207e0_f64) * t943 * t10753 + F::cast_from(0.10254018858216406658e4_f64) * t10756 * t10757 + F::cast_from(3.0_f64) * t10760 * t933 + F::cast_from(3.0_f64) * t2856 * t2881 + F::cast_from(0.96491876992155210402e2_f64) * t10765 * t2889;
    t10768
}
