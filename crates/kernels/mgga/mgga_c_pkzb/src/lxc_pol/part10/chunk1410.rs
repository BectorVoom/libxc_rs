//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1410/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1410<F: Float>(t3880: F, t937: F, t2393: F, t10121: F, t10309: F, t10331: F, t10335: F, t10341: F, t19106: F, t19115: F, t22007: F, t2363: F, t2439: F, t2446: F, t2447: F, t27226: F, t28272: F, t28424: F, t28445: F, t28456: F, t2970: F, t3199: F, t3260: F, t3269: F, t3270: F, t3273: F, t6566: F, t7832: F, t8430: F, t8507: F, t8533: F, t8539: F) -> (F,) {
    let t28492 = t937 * t3880;
    let t28493 = t2393 * t28492;
    let t28516 = 0.13170898365871023197e1 * t2439 * t10341 + 0.26341796731742046394e1 * t2363 * t28456 * t3260 + 0.15805078039045227836e2 * t8507 * t7832 * t10121 * t3199 - 0.13170898365871023197e1 * t2446 * t28445 * t2447 - 0.26341796731742046394e1 * t2393 * t28424 * t3270 - 0.13170898365871023197e1 * t28493 * t3270 - 0.13170898365871023197e1 * t10331 * t8539 - 0.13170898365871023197e1 * t3269 * t2970 * t28272 + 0.13170898365871023197e1 * t10335 * t6566 + 0.15805078039045227836e2 * t19106 * t10309 * t22007 * t27226 - 0.23707617058567841754e2 * t19115 * t10309 * t22007 * t8430 + 0.13170898365871023197e1 * t3273 * t8533 + 0.26341796731742046394e1 * t2363 * t28492 * t3260;
    (t28516,)
}
