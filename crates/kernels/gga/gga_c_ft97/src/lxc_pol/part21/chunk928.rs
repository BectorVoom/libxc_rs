//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 928/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk928<F: Float>(t22755: F, t4474: F, t4491: F, t5522: F, t1669: F, t22522: F, t22534: F, t22549: F, t22583: F, t22619: F, t22743: F, t22761: F, t25750: F, t29469: F, t29474: F, t29479: F, t29483: F, t29486: F, t29490: F, t29494: F, t29498: F, t29503: F, t29506: F, t29510: F, t29515: F, t5538: F, t5540: F, t5569: F, t5570: F, t5579: F, t5598: F) -> (F, F, F) {
    let t29520 = t22755 * t4474;
    let t29523 = t5522 * t4491;
    let t29526 = -0.1721820212247325051e-5 * t5538 * t22743 * t29469 - 0.51690243689028715488e-5 * t5538 * t5540 * t29474 + 0.14846767889314528222e-3 * t22583 * t29479 - 0.51789017496114396277e-5 * t29483 * t22549 - 0.44540303667943584666e-4 * t5569 * t5570 * t29486 + 0.22270151833971792333e-3 * t5569 * t5570 * t29490 - 0.14836531933660919214e-4 * t22534 * t5570 * t29494 + 0.25537443351851851852e-1 * t22522 * t5570 * t29498 - 0.89080607335887169332e-3 * t22619 * t29503 - 0.11491849508333333333e0 * t22761 * t5579 * t29506 + 0.38306165027777777778e-1 * t5598 * t5579 * t29510 + 0.25845121844514357744e-4 * t5538 * t5540 * t29515 - 0.42562405586419753086e-2 * t25750 + 4.0 * t1669 * t29520 - 2.0 * t1669 * t29523;
    (t29520, t29523, t29526)
}
