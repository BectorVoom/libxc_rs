//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1180/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1180<F: Float>(t100541: F, t100558: F, t101098: F, t101107: F, t101139: F, t101145: F, t101150: F, t101161: F, t11121: F, t11313: F, t1290: F, t1669: F, t17839: F, t2035: F, t22548: F, t22568: F, t22613: F, t22619: F, t22711: F, t22755: F, t23728: F, t25649: F, t25653: F, t25722: F, t25752: F, t25753: F, t25756: F, t3076: F, t36368: F, t37550: F, t38150: F, t401: F, t411: F, t428: F, t45886: F, t45890: F, t5536: F, t5538: F, t5540: F, t5569: F, t5579: F, t5598: F, t58882: F, t6434: F, t6445: F, t72: F, t7202: F, t7837: F, t7854: F, t7867: F, t92429: F, t92883: F, t93178: F, t938: F) -> (F,) {
    let t101170 = -0.10357803499222879255e-4 * t25753 * t101098 - 0.51789017496114396277e-5 * t45890 * t25752 * t25756 + 0.25876656037945937584e-6 * t45886 * t25752 * t25756 + 0.60102574844279699039e-6 * t22548 * t101107 - 0.17782141943527538963e-1 * t1669 * t37550 * t1290 * t58882 + 0.474190451827401039e-1 * t1669 * t22755 * t938 * t7854 - 0.20676097475611486196e-4 * t7837 * t5536 * t17839 * t100541 * t100558 - 0.11877414311451622578e-2 * t5569 * t22568 * t25722 - 0.23754828622903245156e-2 * t22613 * t411 * t25649 + 0.23754828622903245156e-2 * t22619 * t411 * t25653 + 0.51690243689028715488e-4 * t38150 * t6434 + 0.37454916916049382717e0 * t5598 * t92429 * t6445 - t101139 + 0.38306165027777777778e-1 * t5598 * t5579 * t72 * t11313 + 0.25845121844514357744e-4 * t5538 * t5540 * t101145 - 0.12020514968855939808e-5 * t11121 * t101150 - 0.89591295428265718861e-3 * t7867 * t2035 * t23728 * t938 + 0.38482339615903025572e-7 * t3076 * t92883 * t22711 * t938 + 0.2108030480665075738e-3 * t36368 * t93178 * t101161 * t401 - 0.2108030480665075738e-3 * t7202 * t93178 * t101161 * t428;
    (t101170,)
}
