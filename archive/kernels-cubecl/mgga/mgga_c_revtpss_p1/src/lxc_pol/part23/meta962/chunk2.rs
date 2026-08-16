//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3252/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3252<F: Float>(t125: F, t22809: F, t5658: F, t9994: F, t1882: F, t6816: F, t22953: F, t1398: F, t46478: F, t1353: F, t13783: F, t13784: F, t13789: F, t13804: F, t1399: F, t22046: F, t22852: F, t23037: F, t3934: F, t3936: F, t3938: F, t47264: F, t48759: F, t5671: F, t5673: F, t5675: F, t6869: F, t73778: F, t73781: F, t73787: F, t73789: F, t73798: F, t73800: F, t73820: F, t73847: F, t85553: F) -> (F, F, F, F, F) {
    let t85563 = t125 * t22809;
    let t85580 = t9994 * t5658;
    let t85585 = t6816 * t1882;
    let t85609 = t125 * t22953;
    let t85614 = t46478 * t1398;
    let t85623 = F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t3936 * t85563 * t1399 - F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t5673 * t85553 * t1399 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t3936 * t73847 * t6869 + F::cast_from(0.51448821741683684368e-2_f64) * t13804 * t3936 * t85553 * t47264 - F::cast_from(0.38586616306262763276e-2_f64) * t13804 * t5673 * t22046 * t85580 - F::cast_from(0.51448821741683684366e-2_f64) * t5671 * t13789 * t85585 * t5675 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t13789 * t85585 * t1399 + F::cast_from(0.1543464652250510531e-1_f64) * t13804 * t13789 * t73820 * t13784 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t13783 * t22852 * t1399 - F::cast_from(0.51448821741683684368e-2_f64) * t5671 * t13789 * t23037 * t1882 * t1353 + F::cast_from(0.13605355082800796532e0_f64) * t73778 - F::cast_from(0.17149607247227894789e-3_f64) * t73781 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t3936 * t85609 * t3938 + F::cast_from(0.51448821741683684368e-2_f64) * t48759 * t5673 * t85553 * t85614 - F::cast_from(0.24009450146119052704e-1_f64) * t73787 - F::cast_from(0.32524801797942610064e-2_f64) * t73789 - F::cast_from(0.24009450146119052704e-1_f64) * t73798 + F::cast_from(0.12004725073059526352e0_f64) * t73800;
    (t85580, t85585, t85609, t85614, t85623)
}
