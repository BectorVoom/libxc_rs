//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2288/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2288(t17948: f64, t460: f64, t12050: f64, t3601: f64, t471: f64, t17710: f64, t1204: f64, t5462: f64, t3754: f64, t5219: f64, t1234: f64, t12699: f64, t12709: f64, t12717: f64, t12723: f64, t1285: f64, t17331: f64, t1770: f64, t17917: f64, t17921: f64, t17934: f64, t17941: f64, t17945: f64, t1822: f64, t3670: f64, t3746: f64, t3756: f64, t3770: f64, t3774: f64, t3778: f64, t3787: f64, t490: f64, t5436: f64, t5446: f64, t5466: f64, t5470: f64, t5491: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17949 = t460 * t17948;
    let t17951 = t12050 * t3601 * t471;
    let t17952 = t17710 * t17951;
    let t17955 = t1204 * t5462;
    let t17958 = t5219 * t3754;
    let t17961 = 0.65854491829355115987e0_f64 * t12699 * t1822 + 0.65854491829355115987e0_f64 * t5436 * t3778 + 0.13170898365871023197e1_f64 * t3670 * t17917 + 0.65854491829355115987e0_f64 * t1285 * t17921 - 0.13170898365871023197e1_f64 * t12709 * t5446 - 0.13170898365871023197e1_f64 * t12723 * t5446 + 0.13170898365871023197e1_f64 * t3746 * t5491 + 0.13170898365871023197e1_f64 * t3746 * t5470 + 0.65854491829355115987e0_f64 * t1770 * t3787 + 0.13170898365871023197e1_f64 * t17934 * t3770 + 0.65854491829355115987e0_f64 * t17331 * t490 + 0.13170898365871023197e1_f64 * t5436 * t3774 - 0.13170898365871023197e1_f64 * t1234 * t17941 + 0.13170898365871023197e1_f64 * t12717 * t17945 + 0.65854491829355115987e0_f64 * t17949 * t17952 + 0.26341796731742046394e1_f64 * t17955 * t5466 - 0.13170898365871023197e1_f64 * t17958 * t3756;
    (t17949, t17951, t17952, t17955, t17958, t17961)
}
