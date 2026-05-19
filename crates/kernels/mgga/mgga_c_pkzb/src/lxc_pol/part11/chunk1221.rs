//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1221/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1221<F: Float>(t10932: F, t179: F, t18204: F, t299: F, t10981: F, t2099: F, t757: F, t10767: F, t10986: F, t11019: F, t11028: F, t11034: F, t11070: F, t17787: F, t18110: F, t18154: F, t2003: F, t2031: F, t2039: F, t2104: F, t2105: F, t25337: F, t26413: F, t26423: F, t26426: F, t26430: F, t26440: F, t26494: F, t2774: F, t2899: F, t2922: F, t2931: F, t2976: F, t300: F, t30002: F, t30013: F, t302: F, t3650: F, t3679: F, t5693: F, t5729: F, t5984: F, t759: F, t761: F, t7700: F, t7736: F, t7742: F, t9259: F, t9277: F, t9562: F, t9575: F) -> F {
    let t30099 = t299 * t179 * t18204 * t10932;
    let t30144 = t757 * t2099 * t10981;
    let t30156 = -F::cast_from(0.34299214494455789579e-2_f64) * t30099 + F::cast_from(0.38586616306262763276e-2_f64) * t2104 * t300 * t2003 * t3650 * t9259 + F::cast_from(0.77173232612525526549e-2_f64) * t2899 * t30002 * t2031 * t2774 + F::cast_from(0.42874018118069736972e-3_f64) * t26413 + F::cast_from(0.12862205435420921092e-2_f64) * t2922 * t2105 * t3679 * t9575 - F::cast_from(0.64311027177104605458e-3_f64) * t2922 * t302 * t26494 * t11028 - F::cast_from(0.25724410870841842184e-2_f64) * t7736 * t2105 * t11070 * t17787 + F::cast_from(0.38586616306262763275e-2_f64) * t2104 * t5693 * t2976 * t11019 - F::cast_from(0.64311027177104605458e-3_f64) * t2922 * t302 * t9562 * t9277 + F::cast_from(0.68598428988911579157e-2_f64) * t5984 * t11034 - F::cast_from(0.42874018118069736972e-3_f64) * t2104 * t2105 * t10767 * t759 * t761 - F::cast_from(0.68598428988911579157e-2_f64) * t18154 * t10986 - F::cast_from(0.1270341277572436651e-3_f64) * t18110 + F::cast_from(0.25724410870841842184e-2_f64) * t26423 - F::cast_from(0.85748036236139473944e-3_f64) * t26426 + F::cast_from(0.14291339372689912324e-3_f64) * t30144 + F::cast_from(0.25724410870841842184e-2_f64) * t2922 * t7700 * t2039 * t30013 + F::cast_from(0.7717323261252552655e-2_f64) * t7742 * t25337 * t5729 * t2931 + F::cast_from(0.85748036236139473944e-3_f64) * t26430 - F::cast_from(0.85748036236139473944e-3_f64) * t26440;
    t30156
}
