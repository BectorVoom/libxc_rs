//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1220/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1220<F: Float>(t10989: F, t2099: F, t5725: F, t11004: F, t2945: F, t10767: F, t2003: F, t179: F, t2068: F, t299: F, t10821: F, t10932: F, t10934: F, t10990: F, t10999: F, t11005: F, t18063: F, t18160: F, t18210: F, t21395: F, t21863: F, t21870: F, t25556: F, t25572: F, t25576: F, t25580: F, t2739: F, t2887: F, t29813: F, t3515: F, t655: F, t758: F, t7607: F, t771: F, t7787: F, t780: F, t9594: F) -> F {
    let t30049 = t5725 * t2099 * t10989;
    let t30077 = t2945 * t2099 * t11004;
    let t30081 = t2003 * t10767;
    let t30093 = t299 * t179 * t2068 * t10767;
    let t30095 = F::new(0.14291339372689912324e-3) * t25556 + F::new(0.68598428988911579157e-2) * t18160 * t10990 - F::new(0.85748036236139473947e-3) * t30049 - F::new(3.0) / F::new(16.0) * t2887 * t21395 * t10821 * t655 - F::new(0.42874018118069736972e-3) * t299 * t179 * t780 * t29813 - F::new(5.0) / F::new(1296.0) * t18063 + F::new(0.28582678745379824648e-3) * t25572 - F::new(0.13719685797782315831e-1) * t25576 + F::new(0.45732285992607719437e-2) * t25580 + F::new(0.25724410870841842184e-1) * t2945 * t758 * t18210 * t10932 * t655 - F::new(0.1543464652250510531e-1) * t2945 * t758 * t9594 * t2739 + F::new(0.38586616306262763276e-2) * t2945 * t758 * t7787 * t3515 + F::new(0.25724410870841842184e-2) * t30077 - F::new(0.20579528696673473747e-1) * t7607 * t11005 + F::new(0.12862205435420921092e-2) * t2945 * t758 * t30081 * t655 + t21863 - F::new(0.85748036236139473944e-3) * t21870 + F::new(0.27439371595564631663e-1) * t771 * t10934 + F::new(0.22866142996303859718e-2) * t771 * t10999 - F::new(0.28582678745379824648e-3) * t30093;
    t30095
}
