//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3683/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3683<F: Float>(t225: F, t69636: F, t1234: F, t1269: F, t1285: F, t1287: F, t1288: F, t12966: F, t16776: F, t17170: F, t17307: F, t17815: F, t17934: F, t1818: F, t20721: F, t20900: F, t21082: F, t21538: F, t21565: F, t3666: F, t3670: F, t3751: F, t3759: F, t3782: F, t3783: F, t3787: F, t487: F, t5216: F, t5332: F, t5443: F, t5462: F, t5463: F, t5464: F, t5466: F, t59032: F, t59241: F, t6564: F, t68674: F, t69609: F, t69624: F) -> (F, F) {
    let t69637 = t69636 * t225;
    let t69652 = F::cast_from(0.13170898365871023197e1_f64) * t68674 * t1288 + F::cast_from(0.26341796731742046394e1_f64) * t17934 * t17815 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t487 * t69609 * t1287 - F::cast_from(0.13170898365871023197e1_f64) * t1234 * t3759 * t21082 + F::cast_from(0.52683593463484092788e1_f64) * t3670 * t3759 * t20721 + F::cast_from(0.52683593463484092788e1_f64) * t5216 * t5462 * t5466 - F::cast_from(0.13170898365871023197e1_f64) * t3782 * t69624 * t3783 - F::cast_from(0.26341796731742046394e1_f64) * t3666 * t21538 + F::cast_from(0.26341796731742046394e1_f64) * t17307 * t16776 - F::cast_from(0.13170898365871023197e1_f64) * t59032 * t1818 + F::cast_from(0.52683593463484092788e1_f64) * t59241 * t5443 + F::cast_from(0.13170898365871023197e1_f64) * t69637 * t3751 + F::cast_from(0.26341796731742046394e1_f64) * t12966 * t21565 + F::cast_from(0.26341796731742046394e1_f64) * t5463 * t5332 * t5464 * t17170 + F::cast_from(0.13170898365871023197e1_f64) * t1285 * t1269 * t20900 * t1287 + F::cast_from(0.65854491829355115987e0_f64) * t6564 * t3787;
    (t69637, t69652)
}
