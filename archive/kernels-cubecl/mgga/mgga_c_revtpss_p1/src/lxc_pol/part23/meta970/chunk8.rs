//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3278/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3278<F: Float>(t22813: F, t547: F, t807: F, t9941: F, t1413: F, t22809: F, t13767: F, t1868: F, t2661: F, t74012: F, t13789: F, t13790: F, t21990: F, t22079: F, t47262: F, t49008: F, t49012: F, t49030: F, t49057: F, t5671: F, t5673: F, t73847: F, t74579: F, t74583: F, t74585: F, t74589: F, t74598: F, t74602: F, t74606: F, t85625: F) -> F {
    let t86165 = t807 * t547 * t9941 * t22813;
    let t86169 = t807 * t547 * t1413 * t22809;
    let t86183 = t2661 * t13767 * t74012 * t1868;
    let t86198 = F::cast_from(0.85748036236139473942e-3_f64) * t86165 + F::cast_from(0.28582678745379824648e-4_f64) * t86169 + F::cast_from(0.12862205435420921092e-2_f64) * t5671 * t5673 * t73847 * t13790 + F::cast_from(0.12862205435420921092e-2_f64) * t5671 * t5673 * t22079 * t21990 - F::cast_from(0.5421477899694558815e-4_f64) * t49008 - F::cast_from(0.30492001685571196934e-4_f64) * t49012 - F::cast_from(0.42874018118069736973e-3_f64) * t86183 + F::cast_from(455.0_f64) / F::cast_from(216.0_f64) * t49030 - F::cast_from(0.10289764348336736873e-1_f64) * t5671 * t13789 * t13790 * t85625 + F::cast_from(0.32528867398167352889e-3_f64) * t47262 + F::cast_from(0.17149607247227894789e-3_f64) * t74579 + F::cast_from(0.12862205435420921092e-3_f64) * t74583 - F::cast_from(0.17006693853500995666e-1_f64) * t74585 + F::cast_from(0.42874018118069736972e-3_f64) * t74589 - F::cast_from(0.17149607247227894789e-3_f64) * t74598 - F::cast_from(0.17149607247227894789e-3_f64) * t74602 - F::cast_from(0.85748036236139473944e-4_f64) * t74606 + t49057;
    t86198
}
