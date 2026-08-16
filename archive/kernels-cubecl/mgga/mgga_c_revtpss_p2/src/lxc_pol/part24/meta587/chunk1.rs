//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1825/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1825<F: Float>(t13783: F, t13790: F, t1883: F, t22046: F, t22074: F, t22079: F, t22841: F, t22852: F, t3934: F, t3936: F, t46760: F, t47248: F, t48600: F, t5671: F, t5673: F, t6862: F, t6874: F, t74299: F, t74304: F, t74322: F, t74341: F, t74358: F, t74362: F, t85548: F, t85553: F, t85609: F, t85839: F, t85865: F, t91865: F) -> F {
    let t92123 = -F::cast_from(0.45732285992607719437e-3_f64) * t74299 + F::cast_from(0.15246000842785598467e-4_f64) * t74304 - F::cast_from(0.48018900292238105408e-1_f64) * t85839 - F::cast_from(0.16262400898971305032e-2_f64) * t74322 - F::cast_from(0.18295201011342718161e-3_f64) * t48600 - F::cast_from(0.27107389498472794074e-4_f64) * t74341 - F::cast_from(0.34013387707001991332e-1_f64) * t74358 - F::cast_from(0.30492001685571196935e-4_f64) * t74362 + F::cast_from(0.24009450146119052705e-1_f64) * t85865 - F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t5673 * t85609 * t1883 - F::cast_from(0.10289764348336736873e-1_f64) * t5671 * t3936 * t22074 * t6862 - t46760 - F::cast_from(0.12862205435420921092e-2_f64) * t3934 * t5673 * t22079 * t6874 - F::cast_from(0.20579528696673473746e-1_f64) * t5671 * t3936 * t85553 * t22841 + F::cast_from(0.17149607247227894789e-2_f64) * t5671 * t5673 * t85609 * t13790 - F::cast_from(0.51448821741683684368e-1_f64) * t3934 * t13783 * t22852 * t1883 + F::cast_from(0.10289764348336736874e0_f64) * t3934 * t47248 * t85548 * t1883 + F::cast_from(0.51448821741683684366e-2_f64) * t3934 * t3936 * t22046 * t91865;
    t92123
}
