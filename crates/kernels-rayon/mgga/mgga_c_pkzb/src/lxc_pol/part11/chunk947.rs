//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 947/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk947(t3207: f64, t7832: f64, t3903: f64, t914: f64, t10282: f64, t10310: f64, t10311: f64, t10316: f64, t10319: f64, t10320: f64, t10324: f64, t10331: f64, t10335: f64, t10341: f64, t10344: f64, t10349: f64, t10352: f64, t1250: f64, t2439: f64, t3259: f64, t3260: f64, t3266: f64, t3269: f64, t3270: f64, t3273: f64, t3914: f64, t3920: f64, t3923: f64, t397: f64, t6561: f64, t6579: f64, t8549: f64, t943: f64, t946: f64) -> (f64, f64, f64) {
    let t10353 = t7832 * t3207;
    let t10356 = t914 * t3903;
    let t10361 = 0.39512695097613069591e1_f64 * t10310 * t10311 + 0.13170898365871023197e1_f64 * t6561 * t3914 + 0.26341796731742046394e1_f64 * t3259 * t10316 - 0.39512695097613069591e1_f64 * t10319 * t10320 + 0.26341796731742046394e1_f64 * t10324 * t3260 + 0.13170898365871023197e1_f64 * t8549 * t1250 + 0.13170898365871023197e1_f64 * t3273 * t3266 - 0.13170898365871023197e1_f64 * t10331 * t3270 + 0.13170898365871023197e1_f64 * t10335 * t3260 + 0.65854491829355115987e0_f64 * t2439 * t3920 + 0.65854491829355115987e0_f64 * t943 * t10341 - 0.65854491829355115987e0_f64 * t10344 * t3270 - 0.65854491829355115987e0_f64 * t6579 * t3923 - 0.13170898365871023197e1_f64 * t3269 * t10349 + 0.65854491829355115987e0_f64 * t10352 * t10353 + 0.65854491829355115987e0_f64 * t10356 * t946 + 0.65854491829355115987e0_f64 * t397 * t10282;
    (t10353, t10356, t10361)
}
