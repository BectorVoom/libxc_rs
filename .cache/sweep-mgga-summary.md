# Batched compile sweep — summary

- effective jobs: `1`
- effective batch_size: `20`
- families: `mgga`
- total packages swept: `131`
- pass-1 successes: `131`
- pass-2 successes: `0`
- failures: `0`
- total wall-clock: `15212.9 s`
- peak-of-peak RSS: `26619.1 MB`

## Per-family

| family | ok | pass-2 | fail |
|---|---|---|---|
| mgga | 131 | 0 | 0 |

## Per-batch timing + RSS

| batch_index | package_count | wall_time_s | peak_rss_mb | all_ok |
|---|---|---|---|---|
| 0 | 20 | 1387.9 | 26619.1 | True |
| 1 | 20 | 4022.7 | 18186.8 | True |
| 2 | 20 | 1731.0 | 11672.6 | True |
| 3 | 20 | 1851.5 | 15991.4 | True |
| 4 | 20 | 3631.5 | 15768.4 | True |
| 5 | 20 | 1484.0 | 8674.0 | True |
| 6 | 11 | 1104.4 | 8577.6 | True |

VERDICT: ALL_OK
