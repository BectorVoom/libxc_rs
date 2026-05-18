//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 947/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk947<F: Float>(t3207: F, t7832: F, t3903: F, t914: F, t10282: F, t10310: F, t10311: F, t10316: F, t10319: F, t10320: F, t10324: F, t10331: F, t10335: F, t10341: F, t10344: F, t10349: F, t10352: F, t1250: F, t2439: F, t3259: F, t3260: F, t3266: F, t3269: F, t3270: F, t3273: F, t3914: F, t3920: F, t3923: F, t397: F, t6561: F, t6579: F, t8549: F, t943: F, t946: F) -> (F, F, F) {
    let t10353 = t7832 * t3207;
    let t10356 = t914 * t3903;
    let t10361 = F::new(0.39512695097613069591e1) * t10310 * t10311 + F::new(0.13170898365871023197e1) * t6561 * t3914 + F::new(0.26341796731742046394e1) * t3259 * t10316 - F::new(0.39512695097613069591e1) * t10319 * t10320 + F::new(0.26341796731742046394e1) * t10324 * t3260 + F::new(0.13170898365871023197e1) * t8549 * t1250 + F::new(0.13170898365871023197e1) * t3273 * t3266 - F::new(0.13170898365871023197e1) * t10331 * t3270 + F::new(0.13170898365871023197e1) * t10335 * t3260 + F::new(0.65854491829355115987e0) * t2439 * t3920 + F::new(0.65854491829355115987e0) * t943 * t10341 - F::new(0.65854491829355115987e0) * t10344 * t3270 - F::new(0.65854491829355115987e0) * t6579 * t3923 - F::new(0.13170898365871023197e1) * t3269 * t10349 + F::new(0.65854491829355115987e0) * t10352 * t10353 + F::new(0.65854491829355115987e0) * t10356 * t946 + F::new(0.65854491829355115987e0) * t397 * t10282;
    (t10353, t10356, t10361)
}
