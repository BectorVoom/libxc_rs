//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 805/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk805<F: Float>(t44001: F, t3005: F, t9800: F, t9829: F, t13142: F, t7416: F, t10054: F, t3040: F, t3267: F, t8556: F, t1445: F, t2087: F, t43240: F, t13161: F, t5782: F, t13149: F, t2464: F, t825: F) -> (F, F, F, F, F, F, F, F) {
    let t44002 = 0.15976219147466979032e-1 * t44001;
    let t44004 = t9800 * t3005 * t9829;
    let t44005 = 0.19171462976960374838e1 * t44004;
    let t44009 = t7416 * t13142;
    let t44010 = 0.15976219147466979032e-1 * t44009;
    let t44027 = 0.35750489951850426669e0 * t10054 * t3040;
    let t44029 = 0.23833659967900284446e0 * t3267 * t8556;
    let t44038 = 0.62115540045351614476e2 * t2087 * t1445 * t43240;
    let t44040 = 0.62115540045351614476e2 * t5782 * t13161;
    let t44045 = t825 * t2464 * t13149;
    (t44002, t44005, t44010, t44027, t44029, t44038, t44040, t44045)
}
