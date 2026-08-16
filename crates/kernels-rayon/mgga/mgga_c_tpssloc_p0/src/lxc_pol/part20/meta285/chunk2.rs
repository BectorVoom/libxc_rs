//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1482/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1482(t10717: f64, t10720: f64, t10724: f64, t10729: f64, t10733: f64, t10734: f64, t10739: f64, t10740: f64, t10744: f64, t10747: f64, t10750: f64, t10753: f64, t10756: f64, t10757: f64, t10760: f64, t10765: f64, t2856: f64, t2861: f64, t2863: f64, t2881: f64, t2886: f64, t2889: f64, t2905: f64, t2907: f64, t2930: f64, t933: f64, t943: f64) -> f64 {
    let t10768 = 0.96491876992155210402e2_f64 * t2886 * t10717 - 0.35089341735807877242e1_f64 * t2905 * t10720 + 0.51947577317044391277e2_f64 * t2930 * t10724 + t10729 - t10733 - 6.0_f64 * t2861 * t10734 - t10739 - 6.0_f64 * t10740 * t2863 + 6.0_f64 * t2886 * t10744 - 0.35089341735807877242e1_f64 * t10747 * t2907 + 0.35089341735807877242e1_f64 * t2930 * t10750 + 0.5848223622634646207e0_f64 * t943 * t10753 + 0.10254018858216406658e4_f64 * t10756 * t10757 + 3.0_f64 * t10760 * t933 + 3.0_f64 * t2856 * t2881 + 0.96491876992155210402e2_f64 * t10765 * t2889;
    t10768
}
