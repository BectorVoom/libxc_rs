//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2331/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2331<F: Float>(t477: F, t5052: F, t27654: F, t7327: F, t24745: F, t4935: F, t1090: F, t1186: F, t1201: F, t1215: F, t15771: F, t2121: F, t2147: F, t24589: F, t24799: F, t24849: F, t24851: F, t27406: F, t27525: F, t27549: F, t27552: F, t27722: F, t27732: F, t3966: F, t462: F, t7283: F, t7362: F, t7364: F, t7373: F, t7376: F, t7377: F, t86106: F, t86113: F, t86116: F, t94976: F) -> F {
    let t95794 = t477 * t5052;
    let t95803 = t27654 * t7327;
    let t95813 = t4935 * t24745;
    let t95817 = F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t462 * t2147 * t15771 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t24851 * t3966 * t1215 * t7376 + F::cast_from(2.0_f64) * t1201 * t27722 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t24799 + F::cast_from(0.12184696791468343974e-2_f64) * t86106 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t7362 * t95794 * t1090 - F::cast_from(0.27415567780803773942e-2_f64) * t86113 - F::cast_from(0.73108180748810063846e-2_f64) * t27549 * t94976 * t27552 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t95803 * t7377 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1186 * t27732 - F::cast_from(0.54831135561607547884e-2_f64) * t24849 * t86116 * t27525 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t95813 * t7364;
    t95817
}
