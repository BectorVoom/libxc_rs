//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1314/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1314<F: Float>(t12831: F, t26866: F, t12917: F, t26870: F, t26827: F, t3678: F, t12987: F, t2138: F, t13085: F, t7624: F, t12816: F, t12822: F, t12828: F, t12836: F, t12862: F, t12868: F, t12912: F, t12938: F, t12991: F, t13102: F, t26852: F, t26867: F, t26880: F, t3640: F, t3644: F, t3723: F, t97161: F, t97169: F, t97171: F, t97174: F, t97177: F, t97179: F) -> F {
    let t97182 = t12831 * t26866;
    let t97187 = t26870 * t12917;
    let t97191 = t26827 * t3678;
    let t97193 = t12987 * t2138;
    let t97200 = t7624 * t13085;
    let t97202 = -F::cast_from(0.28582678745379824648e-3_f64) * t7624 * t12822 - F::cast_from(0.17149607247227894789e-2_f64) * t7624 * t12828 + F::cast_from(0.95275595817932748825e-3_f64) * t97161 - F::cast_from(0.1270341277572436651e-2_f64) * t7624 * t13102 - F::cast_from(0.85748036236139473944e-3_f64) * t26867 * t12836 - F::cast_from(0.14291339372689912324e-2_f64) * t26880 * t12938 - F::cast_from(0.85748036236139473944e-3_f64) * t97169 - F::cast_from(0.11433071498151929859e-2_f64) * t97171 + F::cast_from(0.17149607247227894789e-2_f64) * t97174 * t12868 + t97177 / F::new(432.0) + F::cast_from(0.25724410870841842183e-2_f64) * t97179 * t12912 - F::cast_from(0.25724410870841842183e-2_f64) * t97182 * t3723 - F::cast_from(0.12862205435420921092e-2_f64) * t26870 * t12862 - F::cast_from(0.17149607247227894789e-2_f64) * t97187 + F::cast_from(0.28582678745379824648e-2_f64) * t7624 * t12816 - F::cast_from(0.17149607247227894789e-2_f64) * t97191 - F::cast_from(0.25724410870841842183e-2_f64) * t97193 * t12991 - F::cast_from(0.85748036236139473944e-3_f64) * t26852 * t3640 - F::cast_from(0.17149607247227894789e-2_f64) * t26852 * t3644 - F::cast_from(0.57165357490759649295e-3_f64) * t97200;
    t97202
}
