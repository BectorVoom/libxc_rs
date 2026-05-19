//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 695/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk695<F: Float>(t1849: F, t569: F, t1310: F, t10463: F, t662: F, t10441: F, t10449: F, t1776: F, t1775: F, t10781: F, t10785: F, t10787: F, t10789: F, t10795: F, t10800: F, t10804: F, t10810: F, t10814: F, t10817: F, t10821: F, t10828: F, t1773: F, t1778: F, t4989: F, t5009: F, t5013: F, t5022: F, t5026: F, t5034: F) -> (F, F) {
    let t10831 = F::new(1.0) / t569 / t1849;
    let t10832 = t1310 * t10831;
    let t10833 = t662 * t10463;
    let t10834 = t10833 * t10441;
    let t10835 = t10832 * t10834;
    let t10838 = t1776 * t10449;
    let t10839 = t1775 * t10838;
    let t10842 = -F::cast_from(0.5397236614853195164e-1_f64) * t1773 * t10781 + F::cast_from(0.10794473229706390328e0_f64) * t10785 + F::cast_from(0.53972366148531951639e-1_f64) * t10787 - F::cast_from(0.35981577432354634426e-1_f64) * t10789 + t10795 + F::cast_from(0.32383419689119170984e0_f64) * t4989 * t5034 - F::cast_from(0.35981577432354634425e-1_f64) * t10800 - F::cast_from(0.71963154864709268852e-1_f64) * t5013 * t10804 - F::cast_from(0.10794473229706390328e0_f64) * t4989 * t5022 - F::cast_from(0.35981577432354634425e-1_f64) * t10810 - F::cast_from(0.1439263097294185377e0_f64) * t1773 * t10814 + F::cast_from(0.53972366148531951639e-1_f64) * t10817 * t1778 + F::cast_from(0.10794473229706390328e0_f64) * t1773 * t10821 + F::cast_from(0.53972366148531951639e-1_f64) * t4989 * t5026 + F::cast_from(0.71963154864709268852e-1_f64) * t4989 * t5009 + F::cast_from(0.35981577432354634425e-1_f64) * t10828 + F::cast_from(0.55971342672551653552e-1_f64) * t1773 * t10835 + F::cast_from(0.17990788716177317213e-1_f64) * t1773 * t10839;
    (t10832, t10842)
}
