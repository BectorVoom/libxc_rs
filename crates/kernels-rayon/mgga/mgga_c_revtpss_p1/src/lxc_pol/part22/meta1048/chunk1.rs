//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3683/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3683(t225: f64, t69636: f64, t1234: f64, t1269: f64, t1285: f64, t1287: f64, t1288: f64, t12966: f64, t16776: f64, t17170: f64, t17307: f64, t17815: f64, t17934: f64, t1818: f64, t20721: f64, t20900: f64, t21082: f64, t21538: f64, t21565: f64, t3666: f64, t3670: f64, t3751: f64, t3759: f64, t3782: f64, t3783: f64, t3787: f64, t487: f64, t5216: f64, t5332: f64, t5443: f64, t5462: f64, t5463: f64, t5464: f64, t5466: f64, t59032: f64, t59241: f64, t6564: f64, t68674: f64, t69609: f64, t69624: f64) -> (f64, f64) {
    let t69637 = t69636 * t225;
    let t69652 = 0.13170898365871023197e1_f64 * t68674 * t1288 + 0.26341796731742046394e1_f64 * t17934 * t17815 + 0.65854491829355115987e0_f64 * t1285 * t487 * t69609 * t1287 - 0.13170898365871023197e1_f64 * t1234 * t3759 * t21082 + 0.52683593463484092788e1_f64 * t3670 * t3759 * t20721 + 0.52683593463484092788e1_f64 * t5216 * t5462 * t5466 - 0.13170898365871023197e1_f64 * t3782 * t69624 * t3783 - 0.26341796731742046394e1_f64 * t3666 * t21538 + 0.26341796731742046394e1_f64 * t17307 * t16776 - 0.13170898365871023197e1_f64 * t59032 * t1818 + 0.52683593463484092788e1_f64 * t59241 * t5443 + 0.13170898365871023197e1_f64 * t69637 * t3751 + 0.26341796731742046394e1_f64 * t12966 * t21565 + 0.26341796731742046394e1_f64 * t5463 * t5332 * t5464 * t17170 + 0.13170898365871023197e1_f64 * t1285 * t1269 * t20900 * t1287 + 0.65854491829355115987e0_f64 * t6564 * t3787;
    (t69637, t69652)
}
