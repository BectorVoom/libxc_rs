//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1211/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1211<F: Float>(t14: F, t30725: F, t231: F, t27669: F, t30852: F, t1127: F, t668: F, t66323: F, t689: F, t66451: F, t13521: F, t30688: F, t17959: F, t27595: F, t108445: F, t108447: F, t108448: F, t108454: F, t108487: F, t108494: F, t108501: F, t108972: F, t1120: F, t122729: F, t232: F, t2347: F, t2360: F, t24265: F, t24361: F, t24372: F, t27529: F, t27557: F, t27561: F, t27609: F, t27616: F, t27620: F, t27646: F, t30635: F, t30838: F, t3886: F, t6034: F, t684: F, t79942: F, t96451: F, t96616: F, t96750: F, t98545: F) -> (F, F, F, F, F, F, F) {
    let t122737 = t30725 * t14;
    let t122738 = t122737 * t231;
    let t122755 = t30852 * t27669;
    let t122765 = t1127 * t668;
    let t122770 = t66323 * t689;
    let t122774 = t66451 * t689;
    let t122778 = t30688 * t13521;
    let t122782 = t27595 * t17959;
    let t122786 = -t108445 - 0.98910212891072794758e-5 * t108454 + 0.23754828622903245156e-3 * t6034 * t1120 * t27557 + 0.79128170312858235809e-4 * t24372 * t1120 * t27561 + 0.89080607335887169332e-4 * t24265 * t232 * t122729 + 0.60548059007656442387e-3 * t108447 * t108448 * t30838 * t684 - 0.61601711269092797215e-4 * t27616 * t122738 * t27620 - 0.51074886703703703704e-1 * t24361 * t98545 * t1127 * t2360 * t3886 + 0.34049924469135802469e-1 * t24361 * t108972 * t1127 * t2347 * t3886 + t108494 - t108501 - 0.44540303667943584666e-3 * t24265 * t232 * t79942 - 0.17263005832038132092e-5 * t122755 * t96616 + 0.51074886703703703703e-1 * t108487 * t98545 * t30635 * t684 - 0.23754828622903245156e-2 * t27609 * t1120 * t27529 - 0.76612330055555555556e-1 * t96451 * t98545 * t122765 * t27646 - 0.44540303667943584666e-4 * t6034 * t232 * t122770 - 0.14836531933660919214e-4 * t24372 * t232 * t122774 + 0.29673063867321838428e-4 * t96750 * t232 * t122778 - 0.89080607335887169332e-3 * t24265 * t232 * t122782;
    (t122737, t122765, t122770, t122774, t122778, t122782, t122786)
}
