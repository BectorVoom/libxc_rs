//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1356/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1356<F: Float>(t3759: F, t6573: F, t1234: F, t12756: F, t1285: F, t1291: F, t12966: F, t12987: F, t1770: F, t1825: F, t21333: F, t21518: F, t21521: F, t21524: F, t21527: F, t21535: F, t21538: F, t21542: F, t21551: F, t21554: F, t21558: F, t21562: F, t3670: F, t460: F, t490: F, t5216: F, t5478: F, t5494: F, t6564: F, t6714: F) -> F {
    let t21565 = t3759 * t6573;
    let t21568 = F::new(0.13170898365871023197e1) * t12756 * t21518 - F::new(0.39512695097613069591e1) * t12987 * t21521 + F::new(0.26341796731742046394e1) * t3670 * t21524 + F::new(0.65854491829355115987e0) * t460 * t21527 + F::new(0.13170898365871023197e1) * t5216 * t1825 + F::new(0.13170898365871023197e1) * t12966 * t6714 + F::new(0.65854491829355115987e0) * t1285 * t21535 - F::new(0.13170898365871023197e1) * t1234 * t21538 - F::new(0.65854491829355115987e0) * t1234 * t21542 + F::new(0.13170898365871023197e1) * t1770 * t5494 + F::new(0.65854491829355115987e0) * t6564 * t1291 + F::new(0.65854491829355115987e0) * t21333 * t490 - F::new(0.65854491829355115987e0) * t1234 * t21551 - F::new(0.65854491829355115987e0) * t1234 * t21554 - F::new(0.13170898365871023197e1) * t5478 * t21558 + F::new(0.65854491829355115987e0) * t1285 * t21562 + F::new(0.13170898365871023197e1) * t3670 * t21565;
    t21568
}
